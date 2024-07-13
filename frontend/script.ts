document.addEventListener("DOMContentLoaded", () => {
    setInterval(() => {
        const hex = document.getElementById("hex")!;
        const hexInversion = document.getElementById("hex-inversion");

        let rect = hex.getBoundingClientRect();
        let vh = window.innerHeight;
        let columns = rect.width / (2*vh / 2);
    }, 50);
});