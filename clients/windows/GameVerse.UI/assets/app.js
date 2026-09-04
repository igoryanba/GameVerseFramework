"use strict";

const pending = new Map();
const titles = { connection: "Подготовка к игре", character: "Выбор персонажа", world: "Мир и работа", inventory: "Ваши предметы" };
let registration = false;

function request(command, payload = {}) {
  const requestId = crypto.randomUUID();
  window.chrome.webview.postMessage({ schema_version: 1, request_id: requestId, command, payload });
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => { pending.delete(requestId); reject(new Error("Время ожидания истекло")); }, 5000);
    pending.set(requestId, { resolve, reject, timeout });
  });
}

function toast(message) {
  const node = document.getElementById("toast");
  node.textContent = message;
  node.classList.add("show");
  setTimeout(() => node.classList.remove("show"), 3500);
}

window.chrome.webview.addEventListener("message", event => {
  const message = event.data;
  if (!message || message.schema_version !== 1 || typeof message.request_id !== "string") return;
  if (message.request_id === "bridge-stage") {
    setStage(message.payload?.stage, message.payload?.message);
    document.getElementById("resume-session").hidden = !message.payload?.has_saved_session;
    return;
  }
  const item = pending.get(message.request_id);
  if (!item) return;
  clearTimeout(item.timeout);
  pending.delete(message.request_id);
  if (message.ok) item.resolve(message.payload); else item.reject(new Error(message.message || message.error_code));
});

document.querySelectorAll(".nav").forEach(button => button.addEventListener("click", () => {
  document.querySelectorAll(".nav,.screen").forEach(node => node.classList.remove("active"));
  button.classList.add("active");
  document.getElementById(button.dataset.screen).classList.add("active");
  document.getElementById("page-title").textContent = titles[button.dataset.screen];
  if (button.dataset.screen === "inventory") loadShop();
}));

document.getElementById("login-form").addEventListener("submit", async event => {
  event.preventDefault();
  const data = new FormData(event.currentTarget);
  const payload = { login: data.get("login"), password: data.get("password") };
  if (registration) payload.invite = data.get("invite");
  try {
    await request(registration ? "auth.register" : "auth.login", payload);
    await loadCharacters();
    showScreen("character");
  }
  catch (error) { toast(error.message); }
  data.delete("password");
});

document.getElementById("show-register").addEventListener("click", () => {
  registration = !registration;
  document.getElementById("invite-field").hidden = !registration;
  document.getElementById("auth-title").textContent = registration ? "Регистрация по инвайту" : "Вход";
  document.getElementById("auth-submit").textContent = registration ? "Создать аккаунт" : "Продолжить";
  document.getElementById("show-register").textContent = registration ? "Вернуться ко входу" : "У меня есть инвайт";
});
document.getElementById("resume-session").addEventListener("click", async () => {
  try { await request("session.reconnect"); await loadCharacters(); showScreen("character"); } catch (error) { toast(error.message); }
});
document.getElementById("character-form").addEventListener("submit", async event => {
  event.preventDefault(); const data = new FormData(event.currentTarget);
  try {
    const result = await request("characters.create", { first_name:data.get("first_name"), last_name:data.get("last_name") });
    renderCharacters(result.characters); event.currentTarget.reset();
  } catch (error) { toast(error.message); }
});
document.getElementById("chat-form").addEventListener("submit", async event => {
  event.preventDefault(); const input = event.currentTarget.elements.message; if (!input.value.trim()) return;
  try { await request("chat.send", { message: input.value.trim() }); } catch (error) { toast(error.message); }
  input.value = "";
});
document.querySelectorAll("[data-command]").forEach(button => button.addEventListener("click", async () => {
  try { await request(button.dataset.command, {}); } catch (error) { toast(error.message); }
}));

async function loadShop() {
  try {
    const result = await request("shop.catalog");
    const list = document.getElementById("shop-list"); list.replaceChildren();
    result.items.forEach(item => {
      const row = document.createElement("div"); row.className = "shop-item";
      const label = document.createElement("span"); label.textContent = `${item.name} · $${item.price}`;
      const buy = document.createElement("button"); buy.textContent = "Купить";
      buy.addEventListener("click", async () => { try { await request("shop.buy", { item_id: item.item_id, quantity: 1 }); } catch (error) { toast(error.message); } });
      row.append(label, buy); list.append(row);
    });
  } catch (error) { toast(error.message); }
}

document.getElementById("connection-text").textContent = "Интерфейс готов";

async function loadCharacters() {
  const result = await request("characters.list");
  renderCharacters(result.characters);
}

function renderCharacters(characters) {
  const list = document.getElementById("character-list"); list.replaceChildren();
  if (!characters?.length) { const empty=document.createElement("p"); empty.textContent="Создайте первого персонажа."; list.append(empty); return; }
  characters.forEach(character => {
    const row=document.createElement("div"); row.className="character-item";
    const name=document.createElement("div"); const strong=document.createElement("strong"); strong.textContent=`${character.first_name} ${character.last_name}`;
    const id=document.createElement("small"); id.textContent=`ID ${character.id}`; name.append(strong,id);
    const select=document.createElement("button"); select.textContent="Играть";
    select.addEventListener("click",async()=>{ try { await request("characters.select",{character_id:character.id}); showScreen("world"); setStage("active","Персонаж появился в мире"); } catch(error){toast(error.message);} });
    row.append(name,select); list.append(row);
  });
}

function showScreen(name) {
  document.querySelectorAll(".nav,.screen").forEach(node=>node.classList.remove("active"));
  document.querySelector(`.nav[data-screen="${name}"]`)?.classList.add("active");
  document.getElementById(name).classList.add("active");
  document.getElementById("page-title").textContent=titles[name];
}

function setStage(stage,message) {
  document.getElementById("connection-text").textContent=message||stage||"Подключение…";
  document.getElementById("reconnect").hidden=stage!=="reconnecting"&&stage!=="failed";
}

document.getElementById("reconnect").addEventListener("click",async()=>{ try { await request("session.reconnect"); } catch(error){toast(error.message);} });

fetch("locales/ru-RU.json").then(response=>response.json()).then(locale=>{
  document.querySelectorAll("[data-i18n]").forEach(node=>{ const value=node.dataset.i18n.split(".").reduce((current,key)=>current?.[key],locale); if(value)node.textContent=value; });
}).catch(()=>{});
