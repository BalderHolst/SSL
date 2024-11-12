import init, { render, get_buffer_ptr, get_buffer_size, canvas_width, canvas_height } from "./pkg/ssl_demo.js";

let render_to_canvas = undefined;

init().then((wasm) => {

    render_to_canvas = (code) => {
        console.log("Running code: ", code);

        render(code);

        const ptr = get_buffer_ptr();
        const size = get_buffer_size();
        const memory = new Uint8Array(wasm.memory.buffer);

        const canvasElement = document.querySelector("canvas");
        const canvasContext = canvasElement.getContext("2d");

        const width = canvas_width();
        const height = canvas_height();

        const imageData = canvasContext.createImageData(
            width, height
        );

        canvasContext.canvas.width = width;
        canvasContext.canvas.height = height;
        canvasContext.clearRect(0, 0, width, height);

        const imageDataArray = memory.slice(
            ptr,
            ptr + size
        );

        imageData.data.set(imageDataArray);
        canvasContext.putImageData(imageData, 0, 0);
    };
    render_to_canvas("Stupid Shader Language");
});

// Run button
document.getElementById("run").addEventListener("click", () => {
    const code = document.getElementById("code").value;
    if (render_to_canvas) render_to_canvas(code);
});

// Download button
document.getElementById("download").addEventListener("click", () => {
    let uri = document.querySelector("canvas").toDataURL();
    let link = document.createElement("a");
    link.download = "ssl-output.png";
    link.href = uri;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
});

// Run code on ctrl-enter
document.getElementById("code").addEventListener("keydown", (event) => {
    if (event.ctrlKey && event.key === "Enter") {
        const code = document.getElementById("code").value;
        if (render_to_canvas) render_to_canvas(code);
    }
});
