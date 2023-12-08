const targetDate = new Date("2028-05-01T00:00:00.000Z");

export function startCooldownClock() {
    setInterval(renderClock, 10);
}

function renderClock() {
    const container = document.getElementById("countdown-text");

    const now = new Date();
    const diff = targetDate - now;

    const years = Math.floor(diff / (1000 * 60 * 60 * 24 * 365));
    const days = Math.floor(diff / (1000 * 60 * 60 * 24)) % 365;
    const hours = Math.floor(diff / (1000 * 60 * 60)) % 24;
    const minutes = Math.floor(diff / (1000 * 60)) % 60;
    const seconds = Math.floor(diff / (1000)) % 60;
    const milliseconds = Math.floor(diff) % 1000;

    container.innerHTML = `
        <div class="flex">
        ${String(years).padStart(2, '0')} <span class="small-caps lg:text-xl text-sm ">années</span> 
        ${String(days).padStart(3, '0')} <span class="small-caps lg:text-xl text-sm ">jours</span>
        ${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}
        <span class="text-sm">${String(milliseconds).padStart(3, '0')}</span>
        </div>
    `;
}