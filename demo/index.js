import init, { render, get_buffer_ptr, get_buffer_size, canvas_width, canvas_height } from "./pkg/ssl_demo.js";


init().then((wasm) => {
    render();

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

    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

    const imageDataArray = memory.slice(
        ptr,
        ptr + size
    );

    imageData.data.set(imageDataArray);
    canvasContext.putImageData(imageData, 0, 0);


});
