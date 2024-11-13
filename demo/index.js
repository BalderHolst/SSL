import init, { render, get_buffer_ptr, get_buffer_size, canvas_width, canvas_height } from "./pkg/ssl_demo.js";
let render_to_canvas = undefined;

const default_code_background = window.getComputedStyle(document.getElementById("code")).getPropertyValue("background");
init().then((wasm) => {

    render_to_canvas = (code) => {
        console.log("Running code: ", code);

        const codeElement = document.getElementById("code");

        // Set background color to white
        codeElement.style.background = "#211";

        setTimeout(() => {

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

            codeElement.classList.remove("running");

            codeElement.style.background = default_code_background;
        }, 0);
    };

    // Initial render image
    render_to_canvas("Stupid Shader Language");
});

// Run button
const default_code_bg_color = window.getComputedStyle(document.getElementById("code")).getPropertyValue("background");
document.getElementById("run").addEventListener("click", () => {
    const code = document.getElementById("code").value;
    if (render_to_canvas) render_to_canvas(code);
});

// Save button
document.getElementById("save").addEventListener("click", async () => {

    const blob = await new Promise((resolve) => canvas.toBlob(resolve, 'image/png'));

    try {
        // Show the file picker and let the user specify where to save the file
        const fileHandle = await window.showSaveFilePicker({
            suggestedName: 'ssl-output.png',
            types: [
                {
                    description: 'PNG Files',
                    accept: {
                        'image/png': ['.png'],
                    },
                },
            ],
        });

        // Create a writable stream to the file
        const writableStream = await fileHandle.createWritable();

        // Write the content to the file
        await writableStream.write(blob);

        // Close the file and finalize the write operation
        await writableStream.close();

        alert('File saved successfully!');
    } catch (error) {
        console.log("Could not open file picker. Falling back to normal download.");
        let uri = document.querySelector("canvas").toDataURL();
        let link = document.createElement("a");
        link.download = "ssl-output.png";
        link.href = uri;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }

});

// Run code on ctrl-enter
document.getElementById("code").addEventListener("keydown", (event) => {
    if (event.ctrlKey && event.key === "Enter") {
        const code = document.getElementById("code").value;
        if (render_to_canvas) render_to_canvas(code);
    }
});

// Auto grow textarea
document.getElementById("code").addEventListener('input', function () {
    // Save the scroll position
    const scrollPosition = window.scrollY;

    // Adjust the height
    this.style.height = 'auto';
    this.style.height = this.scrollHeight + 'px';

    // Restore the scroll position
    window.scrollTo(0, scrollPosition);
});
