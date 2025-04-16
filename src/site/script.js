document.addEventListener("DOMContentLoaded", imageToBinaryHandler);
document.addEventListener("DOMContentLoaded", binaryToImageHandler);


function imageToBinaryHandler()  {
    const toBinaryBtn = document.querySelector("[action='to-binary-download']")
    const imageInput = document.getElementById("image-input");

    const canvas = document.getElementById("to-binary-canvas");
    const ctx = canvas.getContext("2d");

    imageInput.onchange = (e) => {
        const file = e.target.files[0];
        const reader = new FileReader();

        // Load the image as a Data URL
        reader.onload = () => {
            const img = new Image();
            img.onload = () => {
                // Draw the image onto the canvas
                canvas.width = img.width;
                canvas.height = img.height;
                ctx.drawImage(img, 0, 0);
            };

            // Set the image source to the uploaded file's data URL
            img.src = reader.result;
            toBinaryBtn.disabled = false;
        };

        // Read the uploaded file as a Data URL
        reader.readAsDataURL(file);
    };


    toBinaryBtn.onclick = () => {
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

        const pixelData = imageData.data;
        const binaryData = new Uint8Array(pixelData);

        const blob = new Blob([binaryData], { type: 'application/octet-stream' });
        const url = URL.createObjectURL(blob);

        // Create a downloadable link
        const link = document.createElement('a');
        link.href = url;
        link.download = 'image.bin';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }
}


function binaryToImageHandler () {
    const binaryInput = document.getElementById("binary-input");

    const widthInput = document.getElementById("to-image-width-input");
    const heightInput = document.getElementById("to-image-height-input");

    const toImageConvertBtn = document.querySelector("[action='to-image-convert']");
    const toImageDownloadBtn = document.querySelector("[action='to-image-download']");
    const toImageMsg = document.getElementById("to-image-msg"); 

    const canvas = document.getElementById("to-image-canvas");
    const ctx = canvas.getContext("2d");

    let imageBinary = new Uint8Array();

    binaryInput.onchange = (e) => {
        const file = e.target.files[0];
        const reader = new FileReader();

        // Load the image as a Data URL
        reader.onload = (e) => {
            const arrayBuffer = e.target.result;
            imageBinary = new Uint8Array(arrayBuffer);
            console.log(imageBinary);

            toImageConvertBtn.disabled = false;
        };

        // Read the uploaded file as a Data URL
        reader.readAsArrayBuffer(file);
    }

    toImageConvertBtn.onclick = () => {
        toImageMsg.innerText = "";
        toImageDownloadBtn.disabled = true;

        const widthVal = parseInt(widthInput.value);
        const heightVal = parseInt(heightInput.value);

        if (isNaN(widthVal) || isNaN(heightVal)) {
            toImageMsg.innerText = "Invalid width or height values.";
            return;
        }

        try {
            // Create an ImageData object
            const imageData = new ImageData(new Uint8ClampedArray(imageBinary), widthVal, heightVal);

            // Draw the image data onto the canvas
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            canvas.width = widthVal;
            canvas.height = heightVal;
            ctx.putImageData(imageData, 0, 0);
            toImageDownloadBtn.disabled = false;
       } catch {
            toImageMsg.innerText = "Width / height values do not match the image data.";
            return;
        }
    }


    toImageDownloadBtn.onclick = () => {
        const dataURL = canvas.toDataURL("image/png");
        const link = document.createElement('a');
        link.href = dataURL;
        link.download = 'image.png';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }
}
