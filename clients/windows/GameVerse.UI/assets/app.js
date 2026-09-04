"use strict";

const pending = new Map();
const titles = { connection: "Подготовка к игре", character: "Выбор персонажа", world: "Мир и работа", inventory: "Ваши предметы" };
let registration = false;
let currentStage = "connecting";

function request(command, payload = {}) {
  const requestId = crypto.randomUUID();
  window.chrome.webview.postMessage({ schema_version: 1, request_id: requestId, command, payload });
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => { pending.delete(requestId); reject(new Error("Время ожидания истекло")); }, 5000);
    pending.set(requestId, { resolve, reject, timeout });
  });
}

async function busy(control, operation) {
  if (control?.disabled) return;
  if (control) control.disabled = true;
  try { return await operation(); }
  finally { if (control) control.disabled = false; }
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
  if (button.dataset.screen === "inventory") { loadShop(); loadInventory().catch(error=>toast(error.message)); }
}));

document.getElementById("login-form").addEventListener("submit", async event => busy(event.submitter, async () => {
  event.preventDefault();
  const data = new FormData(event.currentTarget);
  const payload = { login: data.get("login"), password: data.get("password") };
  if (registration) payload.invite = data.get("invite");
  try {
    await request(registration ? "auth.register" : "auth.login", payload);
    document.getElementById("logout").hidden=false;
    await loadCharacters();
    showScreen("character");
  }
  catch (error) { toast(error.message); }
  event.currentTarget.elements.password.value = "";
}));

document.getElementById("show-register").addEventListener("click", () => {
  registration = !registration;
  document.getElementById("invite-field").hidden = !registration;
  document.getElementById("auth-title").textContent = registration ? "Регистрация по инвайту" : "Вход";
  document.getElementById("auth-submit").textContent = registration ? "Создать аккаунт" : "Продолжить";
  document.getElementById("show-register").textContent = registration ? "Вернуться ко входу" : "У меня есть инвайт";
});
document.getElementById("resume-session").addEventListener("click", async () => {
  try { await request("session.reconnect"); document.getElementById("logout").hidden=false; await loadCharacters(); showScreen("character"); } catch (error) { toast(error.message); }
});
document.getElementById("character-form").addEventListener("submit", async event => busy(event.submitter, async () => {
  event.preventDefault(); const data = new FormData(event.currentTarget);
  try {
    const result = await request("characters.create", { first_name:data.get("first_name"), last_name:data.get("last_name") });
    renderCharacters(result.characters); event.currentTarget.reset();
  } catch (error) { toast(error.message); }
}));
document.getElementById("chat-form").addEventListener("submit", async event => {
  event.preventDefault(); const input = event.currentTarget.elements.message; if (!input.value.trim()) return;
  try { const result=await request("chat.send", { message: input.value.trim() }); appendChat(result.message || input.value.trim()); } catch (error) { toast(error.message); }
  input.value = "";
});
document.querySelectorAll("[data-command]").forEach(button => button.addEventListener("click", async () => {
  await busy(button, async()=>{ try {
    const result=await request(button.dataset.command, {});
    if(button.dataset.command==="job.start") setJob(result.active_route||"alpha-route");
    if(button.dataset.command==="job.finish"){ setJob(null); updateWallet(result); }
  } catch (error) { toast(error.message); } });
}));

async function loadShop() {
  try {
    const result = await request("shop.catalog");
    const list = document.getElementById("shop-list"); list.replaceChildren();
    result.items.forEach(item => {
      const row = document.createElement("div"); row.className = "shop-item";
      const label = document.createElement("span"); label.textContent = `${item.name} · $${item.price}`;
      const buy = document.createElement("button"); buy.textContent = "Купить";
      buy.addEventListener("click", async () => busy(buy,async()=>{ try { const result=await request("shop.buy", { item_id: item.item_id, quantity: 1 }); updateWallet(result); await loadInventory(); } catch (error) { toast(error.message); } }));
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
  currentStage=stage||currentStage;
  document.getElementById("connection-text").textContent=message||stage||"Подключение…";
  document.getElementById("reconnect").hidden=stage!=="reconnecting"&&stage!=="failed";
}

function updateWallet(result){
  if(Number.isSafeInteger(result?.cash)) document.getElementById("cash").textContent=`$ ${result.cash}`;
  if(Number.isSafeInteger(result?.bank)) document.getElementById("bank").textContent=`$ ${result.bank}`;
}
function setJob(route){
  document.getElementById("active-job").textContent=route||"Нет";
  document.getElementById("job-start").hidden=!!route;
  document.getElementById("job-finish").hidden=!route;
}
function appendChat(message){
  const list=document.getElementById("chat-list");
  if(list.querySelector("p")) list.replaceChildren();
  const row=document.createElement("p"); row.textContent=message; list.append(row);
}
async function loadInventory(){
  const result=await request("inventory.request");
  const list=document.getElementById("inventory-list"); list.replaceChildren();
  if(!result.items?.length){ const empty=document.createElement("p"); empty.textContent="Инвентарь пуст"; list.append(empty); return; }
  result.items.forEach(item=>{ const row=document.createElement("p"); row.textContent=`Предмет #${item.item_id}: ${item.quantity}`; list.append(row); });
}

document.getElementById("reconnect").addEventListener("click",async()=>{ try { await request("session.reconnect"); } catch(error){toast(error.message);} });
document.getElementById("logout").addEventListener("click",async event=>busy(event.currentTarget,async()=>{
  try { await request("auth.logout"); setStage("auth_required","Войдите в аккаунт"); showScreen("connection"); event.currentTarget.hidden=true; }
  catch(error){toast(error.message);}
}));

fetch("locales/ru-RU.json").then(response=>response.json()).then(locale=>{
  document.querySelectorAll("[data-i18n]").forEach(node=>{ const value=node.dataset.i18n.split(".").reduce((current,key)=>current?.[key],locale); if(value)node.textContent=value; });
}).catch(()=>{});
