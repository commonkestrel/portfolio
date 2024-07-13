document.addEventListener("DOMContentLoaded", () => {
    setInterval(() => {
        const hex = document.getElementById("hex")!;
        const hexInversion = document.getElementById("hex-inversion")!;

        
        const fontHeight = parseFloat(getComputedStyle(hex).fontSize);
        const fontWidth = fontHeight * 3 / 5

        const box = hex.getBoundingClientRect();
        const horizontalChars = Math.floor(box.width / fontWidth);
        const verticalChars = Math.floor(box.height / fontHeight);
        const chars = horizontalChars * verticalChars;

        let text = "";
        for (let i = 0; i < chars; i++) {
            if (i % 3 == 0) {
                text += ' ';
            } else {
                text += randHex();
            }
        }

        hex.innerText = text;
        hexInversion.innerText = text;
    }, 50);
});

const randHex = (): string => {
    const hexRef = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    return hexRef[Math.floor(Math.random() * 16)];
}

const remToPixels = (): number => {
    return parseFloat(getComputedStyle(document.documentElement).fontSize);
}
