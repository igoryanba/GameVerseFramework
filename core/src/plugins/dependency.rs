//! # Система управления зависимостями плагинов
//! 
//! Превосходство над FiveM ресурсами:
//! - Автоматическое разрешение зависимостей
//! - Обнаружение циклических зависимостей
//! - Версионирование с semver
//! - Валидация совместимости

use std::collections::{HashMap, HashSet};
use semver::{Version, VersionReq};
use crate::plugins::{PluginInfo, PluginError, PluginResult};

#[cfg(test)]
use crate::plugins::PluginDependency;

/// Граф зависимостей плагинов
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Узлы графа (плагины)
    nodes: HashMap<String, PluginNode>,
    /// Ребра графа (зависимости)
    edges: HashMap<String, Vec<String>>,
}

/// Узел графа зависимостей
#[derive(Debug, Clone)]
pub struct PluginNode {
    /// Информация о плагине
    pub info: PluginInfo,
    /// Зависит от этих плагинов
    pub depends_on: Vec<String>,
    /// От этого плагина зависят
    pub dependents: Vec<String>,
}

impl DependencyGraph {
    /// Создать новый граф зависимостей
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Добавить плагин в граф
    pub fn add_plugin(&mut self, plugin_info: &PluginInfo) -> PluginResult<()> {
        let plugin_id = &plugin_info.id;
        
        // Проверить дублирование
        if self.nodes.contains_key(plugin_id) {
            return Err(PluginError::DependencyError {
                reason: format!("Plugin {} already exists in dependency graph", plugin_id),
            });
        }
        
        // Создать узел
        let mut depends_on = Vec::new();
        for dep in &plugin_info.dependencies {
            depends_on.push(dep.name.clone());
        }
        
        let node = PluginNode {
            info: plugin_info.clone(),
            depends_on: depends_on.clone(),
            dependents: Vec::new(),
        };
        
        // Добавить узел
        self.nodes.insert(plugin_id.clone(), node);
        
        // Инициализировать ребра для нового плагина
        self.edges.entry(plugin_id.clone()).or_insert_with(Vec::new);
        
        // Добавить ребра от зависимостей к этому плагину
        for dep_name in &depends_on {
            self.edges.entry(dep_name.clone())
                .or_insert_with(Vec::new)
                .push(plugin_id.clone());
        }
        
        // Обновить dependents в других узлах
        for dep_name in depends_on {
            if let Some(dep_node) = self.nodes.get_mut(&dep_name) {
                dep_node.dependents.push(plugin_id.clone());
            }
        }
        
        // Проверить циклические зависимости
        self.check_circular_dependencies(plugin_id)?;
        
        Ok(())
    }

    /// Удалить плагин из графа
    pub fn remove_plugin(&mut self, plugin_id: &str) -> PluginResult<()> {
        // Проверить, есть ли зависимые плагины
        if let Some(node) = self.nodes.get(plugin_id) {
            if !node.dependents.is_empty() {
                return Err(PluginError::DependencyError {
                    reason: format!(
                        "Cannot remove plugin {}: {} plugin(s) depend on it: {}",
                        plugin_id,
                        node.dependents.len(),
                        node.dependents.join(", ")
                    ),
                });
            }
        }
        
        // Удалить узел
        if let Some(node) = self.nodes.remove(plugin_id) {
            // Удалить ребра от этого плагина
            self.edges.remove(plugin_id);
            
            // Удалить ребра от зависимостей к этому плагину
            for dep_name in &node.depends_on {
                if let Some(edges) = self.edges.get_mut(dep_name) {
                    edges.retain(|id| id != plugin_id);
                }
            }
            
            // Обновить dependents в других узлах
            for dep_name in &node.depends_on {
                if let Some(dep_node) = self.nodes.get_mut(dep_name) {
                    dep_node.dependents.retain(|id| id != plugin_id);
                }
            }
        }
        
        Ok(())
    }

    /// Получить порядок загрузки плагинов (топологическая сортировка)
    pub fn get_load_order(&self, plugin_ids: &[String]) -> PluginResult<Vec<String>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        
        for plugin_id in plugin_ids {
            if !visited.contains(plugin_id) {
                self.dfs_topological_sort(plugin_id, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        result.reverse();
        Ok(result)
    }

    /// Проверить зависимости плагина
    pub fn validate_dependencies(&self, plugin_info: &PluginInfo) -> PluginResult<()> {
        for dependency in &plugin_info.dependencies {
            // Проверить существование зависимости
            let dep_node = self.nodes.get(&dependency.name)
                .ok_or_else(|| PluginError::DependencyError {
                    reason: format!("Dependency {} not found", dependency.name),
                })?;
            
            // Проверить версию
            if !self.is_version_compatible(&dependency.version, &dep_node.info.version)? {
                return Err(PluginError::DependencyError {
                    reason: format!(
                        "Version incompatibility: {} requires {} v{}, but v{} is loaded",
                        plugin_info.id,
                        dependency.name,
                        dependency.version,
                        dep_node.info.version
                    ),
                });
            }
        }
        
        Ok(())
    }

    /// Получить зависимые плагины
    pub fn get_dependents(&self, plugin_id: &str) -> PluginResult<Vec<String>> {
        Ok(self.nodes.get(plugin_id)
            .map(|node| node.dependents.clone())
            .unwrap_or_default())
    }

    /// Получить зависимости плагина
    pub fn get_dependencies(&self, plugin_id: &str) -> Vec<String> {
        self.nodes.get(plugin_id)
            .map(|node| node.depends_on.clone())
            .unwrap_or_default()
    }

    /// Проверить циклические зависимости
    fn check_circular_dependencies(&self, start_plugin: &str) -> PluginResult<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        if self.has_cycle_dfs(start_plugin, &mut visited, &mut rec_stack) {
            return Err(PluginError::DependencyError {
                reason: format!("Circular dependency detected involving plugin: {}", start_plugin),
            });
        }
        
        Ok(())
    }

    /// DFS для обнаружения циклов
    fn has_cycle_dfs(&self, node: &str, visited: &mut HashSet<String>, rec_stack: &mut HashSet<String>) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        
        if let Some(edges) = self.edges.get(node) {
            for neighbor in edges {
                if !visited.contains(neighbor) {
                    if self.has_cycle_dfs(neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(node);
        false
    }

    /// DFS для топологической сортировки
    fn dfs_topological_sort(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        temp_visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> PluginResult<()> {
        if temp_visited.contains(node) {
            return Err(PluginError::DependencyError {
                reason: format!("Circular dependency detected at node: {}", node),
            });
        }
        
        if visited.contains(node) {
            return Ok(());
        }
        
        temp_visited.insert(node.to_string());
        
        if let Some(edges) = self.edges.get(node) {
            for neighbor in edges {
                self.dfs_topological_sort(neighbor, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());
        
        Ok(())
    }

    /// Проверить совместимость версий
    fn is_version_compatible(&self, requirement: &str, available: &str) -> PluginResult<bool> {
        let req = VersionReq::parse(requirement)
            .map_err(|e| PluginError::DependencyError {
                reason: format!("Invalid version requirement '{}': {}", requirement, e),
            })?;
        
        let ver = Version::parse(available)
            .map_err(|e| PluginError::DependencyError {
                reason: format!("Invalid version '{}': {}", available, e),
            })?;
        
        Ok(req.matches(&ver))
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_plugin(id: &str, version: &str, deps: Vec<(&str, &str)>) -> PluginInfo {
        let dependencies = deps.into_iter()
            .map(|(name, version)| PluginDependency {
                name: name.to_string(),
                version: version.to_string(),
                required: true,
            })
            .collect();

        PluginInfo {
            id: id.to_string(),
            name: id.to_string(),
            version: version.to_string(),
            author: "Test".to_string(),
            description: "Test plugin".to_string(),
            website: None,
            license: None,
            dependencies,
            tags: Vec::new(),
        }
    }

    #[test]
    fn test_add_plugin() {
        let mut graph = DependencyGraph::new();
        let plugin = create_test_plugin("test", "1.0.0", vec![]);
        
        assert!(graph.add_plugin(&plugin).is_ok());
        assert!(graph.nodes.contains_key("test"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = DependencyGraph::new();
        
        let plugin_a = create_test_plugin("a", "1.0.0", vec![("b", "1.0.0")]);
        let plugin_b = create_test_plugin("b", "1.0.0", vec![("a", "1.0.0")]);
        
        assert!(graph.add_plugin(&plugin_a).is_ok());
        assert!(graph.add_plugin(&plugin_b).is_err());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph::new();
        
        let plugin_a = create_test_plugin("a", "1.0.0", vec![]);
        let plugin_b = create_test_plugin("b", "1.0.0", vec![("a", "1.0.0")]);
        let plugin_c = create_test_plugin("c", "1.0.0", vec![("b", "1.0.0")]);
        
        graph.add_plugin(&plugin_a).unwrap();
        graph.add_plugin(&plugin_b).unwrap();
        graph.add_plugin(&plugin_c).unwrap();
        
        let order = graph.get_load_order(&["a".to_string(), "b".to_string(), "c".to_string()]).unwrap();
        assert_eq!(order, vec!["a", "b", "c"]);
    }
} 