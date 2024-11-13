import init, {
    render,
    get_buffer_ptr,
    get_buffer_size,
    canvas_width,
    canvas_height,
    canvas_aspect_ratio,
    canvas_resolution,
    aspect_ratio_strings,
    dim_strings,
} from "./pkg/ssl_demo.js";

// Get references to the dropdowns and output areas
const ratioSelect = document.getElementById('aspect-ratio');
const dimSelect = document.getElementById('resolution');

let wasm_loaded = false;

function render_to_canvas(code) {
    if (!wasm_loaded) return;
    const ratio = aspectRatios.indexOf(ratioSelect.value);
    const resolution = dimensions(ratio).indexOf(dimSelect.value);
    _render_to_canvas(code, ratio, resolution);
}

let _render_to_canvas = undefined;
let defaultRatio = undefined;
let defaultDim = undefined;
let aspectRatios = undefined;
let dimensions = undefined;

// Run button
const default_code_bg_color = window.getComputedStyle(document.getElementById("code")).getPropertyValue("background");
document.getElementById("run").addEventListener("click", () => {
    const code = document.getElementById("code").value;
    render_to_canvas(code);
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
        render_to_canvas(code);
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

ratioSelect.addEventListener('change', () => {
    const dims = dimensions(aspectRatios.indexOf(ratioSelect.value));
    const def = Math.floor(dims.length / 2);
    populateDropdown(dimSelect, dims, dims[def]);
});

function populateDropdown(selectElement, options, defaultValue) {
    selectElement.innerHTML = '';
    options.forEach(optionValue => {
        const option = document.createElement('option');
        option.value = optionValue;
        option.textContent = optionValue;
        if (optionValue === defaultValue) option.selected = true;
        selectElement.appendChild(option);
    });
}

const default_code_background = window.getComputedStyle(document.getElementById("code")).getPropertyValue("background");
init().then((wasm) => {

    defaultRatio = canvas_aspect_ratio();
    defaultDim = canvas_resolution();
    aspectRatios = aspect_ratio_strings();
    dimensions = (r) => dim_strings(r);

    const initDims = dimensions(defaultRatio);

    populateDropdown(ratioSelect, aspectRatios, aspectRatios[defaultRatio]);
    populateDropdown(dimSelect, initDims, initDims[defaultDim])

    _render_to_canvas = (code, aspect_ratio, resolution) => {
        console.log("Running code: ", code);

        const codeElement = document.getElementById("code");

        // Set background color to white
        codeElement.style.background = "#211";

        setTimeout(() => {

            render(code, aspect_ratio, resolution);

            const width = canvas_width();
            const height = canvas_height();

            const ptr = get_buffer_ptr();
            const size = get_buffer_size();

            const canvasElement = document.querySelector("canvas");
            const canvasContext = canvasElement.getContext("2d");

            const imageData = canvasContext.createImageData(
                width, height
            );

            canvasContext.canvas.width = width;
            canvasContext.canvas.height = height;

            const imageDataArray = new Uint8Array(wasm.memory.buffer, ptr, size);
            imageData.data.set(imageDataArray);

            canvasContext.putImageData(imageData, 0, 0);

            codeElement.classList.remove("running");

            codeElement.style.background = default_code_background;

        }, 0);
    };

    wasm_loaded = true;

    // Initial render image
    render_to_canvas("Stupid Shader Language");
});
