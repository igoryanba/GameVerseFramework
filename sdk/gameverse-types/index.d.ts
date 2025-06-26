// GameVerse Framework TypeScript Definitions
// Core API types and interfaces for server and client development

// Basic types
interface Vector3 {
    x: number;
    y: number;
    z: number;
}

interface Vector4 extends Vector3 {
    w: number;
}

// Entity types
type Entity = number;
type Player = number;
type Vehicle = number;
type Ped = number;
type GameObject = number;  // Renamed to avoid conflict with built-in Object
type Hash = number;

// Event system
type EventHandler = (...args: any[]) => void;
type NetworkEventHandler = (source: string, ...args: any[]) => void;

// Resource info
interface ResourceInfo {
    name: string;
    version: string;
    author: string;
    description: string;
}

// Server-side API
declare module '@gameverse/server' {
    /**
     * Register a network event handler
     * @param eventName - Name of the event to listen for
     * @param handler - Function to handle the event
     */
    export function onNet(eventName: string, handler: NetworkEventHandler): void;

    /**
     * Emit an event to specific client(s)
     * @param eventName - Name of the event to emit
     * @param target - Target client ID or 'all' for broadcast
     * @param data - Data to send with the event
     */
    export function emit(eventName: string, target: string | number, ...data: any[]): void;

    /**
     * Get list of connected players
     * @returns Array of player IDs
     */
    export function getPlayers(): string[];

    /**
     * Get player name by ID
     * @param playerId - Player ID
     * @returns Player name or null
     */
    export function getPlayerName(playerId: string): string | null;

    /**
     * Get current resource info
     * @returns Resource information
     */
    export function getResourceInfo(): ResourceInfo;

    /**
     * Execute database query
     * @param query - SQL query string
     * @param params - Query parameters
     * @returns Promise with query results
     */
    export function dbQuery(query: string, params?: any[]): Promise<any[]>;

    /**
     * Log message to server console
     * @param level - Log level (info, warn, error, debug)
     * @param message - Message to log
     */
    export function log(level: 'info' | 'warn' | 'error' | 'debug', message: string): void;

    /**
     * Register a console command
     * @param command - Command name
     * @param handler - Command handler function
     */
    export function registerCommand(command: string, handler: (source: string, args: string[]) => void): void;
}

// Client-side API  
declare module '@gameverse/client' {
    /**
     * Register a network event handler
     * @param eventName - Name of the event to listen for
     * @param handler - Function to handle the event
     */
    export function onNet(eventName: string, handler: EventHandler): void;

    /**
     * Emit an event to server
     * @param eventName - Name of the event to emit
     * @param data - Data to send with the event
     */
    export function emitServer(eventName: string, ...data: any[]): void;

    /**
     * Get local player ID
     * @returns Local player entity ID
     */
    export function getLocalPlayer(): Player;

    /**
     * Get player position
     * @param player - Player entity (optional, defaults to local player)
     * @returns Player position as Vector3
     */
    export function getPlayerPosition(player?: Player): Vector3;

    /**
     * Set player position
     * @param player - Player entity
     * @param position - New position
     */
    export function setPlayerPosition(player: Player, position: Vector3): void;

    /**
     * Show notification to player
     * @param message - Notification message
     * @param type - Notification type
     */
    export function showNotification(message: string, type?: 'info' | 'success' | 'warning' | 'error'): void;

    /**
     * Register key binding
     * @param key - Key code or name
     * @param handler - Key press handler
     */
    export function registerKeyBinding(key: string | number, handler: () => void): void;
}

// Shared API (available on both server and client)
declare module '@gameverse/shared' {
    /**
     * Create Vector3 object
     * @param x - X coordinate
     * @param y - Y coordinate  
     * @param z - Z coordinate
     * @returns Vector3 object
     */
    export function vector3(x: number, y: number, z: number): Vector3;

    /**
     * Calculate distance between two points
     * @param pos1 - First position
     * @param pos2 - Second position
     * @returns Distance in game units
     */
    export function getDistance(pos1: Vector3, pos2: Vector3): number;

    /**
     * Delay execution
     * @param ms - Milliseconds to wait
     * @returns Promise that resolves after delay
     */
    export function wait(ms: number): Promise<void>;

    /**
     * Generate random number
     * @param min - Minimum value
     * @param max - Maximum value
     * @returns Random number
     */
    export function random(min: number, max: number): number;

    /**
     * Format string with arguments
     * @param template - String template
     * @param args - Arguments to insert
     * @returns Formatted string
     */
    export function format(template: string, ...args: any[]): string;
}

// React hooks for WebAssembly UI (optional)
declare module '@gameverse/react' {
    /**
     * React hook for GameVerse event system
     * @returns Object with emit and onNet functions
     */
    export function useGameVerse(): {
        emit: (eventName: string, ...data: any[]) => void;
        onNet: (eventName: string, handler: EventHandler) => void;
    };

    /**
     * React hook for player data
     * @returns Player data and update functions
     */
    export function usePlayer(): {
        id: string;
        name: string;
        position: Vector3;
        money: number;
        updatePosition: (pos: Vector3) => void;
        updateMoney: (amount: number) => void;
    };

    /**
     * React hook for vehicle data
     * @param vehicleId - Vehicle entity ID
     * @returns Vehicle data
     */
    export function useVehicle(vehicleId: Vehicle): {
        id: Vehicle;
        model: string;
        position: Vector3;
        speed: number;
        health: number;
    };
} 