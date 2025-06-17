// External Libraries
import html2canvas from "html2canvas";
import { jsPDF as JsPDF } from "jspdf";


/**
 * Exports the current content of the .content container as a PDF.
 * Uses html2canvas to capture the DOM content as a canvas,
 * then uses jsPDF to create a multipage PDF with the captured image.
 *
 * @param { document } input - Document to be converted to PDF.
 */
const handleExportPDF = async (input) => {
    // Add class to body to indicate export in progress (for styling)
    document.body.classList.add("exporting");

    try {
        // Capture the content area to a high-resolution canvas
        const canvas = await html2canvas(input, {
            scale: 3,         // increase scale for higher resolution
            useCORS: true,    // allow cross-origin images
            allowTaint: true, // allow tainted canvas for cross-origin images
            letterRendering: true,
            backgroundColor: "#ffffff",
            ignoreElements: (element) => element.classList.contains("empty-row"), // ignore empty placeholder rows
        });

        const pdf = new JsPDF("p", "pt", "a4");
        const pdfWidth = pdf.internal.pageSize.getWidth();
        const pdfHeight = pdf.internal.pageSize.getHeight();
        const padding = 10;

        // Calculate image width and scale to fit page width with padding
        const imgWidth = pdfWidth - padding * 2;
        const scale = imgWidth / canvas.width;

        // Calculate height in pixels that fits on one PDF page after scaling
        const pageHeightPx = pdfHeight / scale;

        let yPos = 0;

        // Loop to create pages if content height exceeds one page
        while (yPos < canvas.height) {
            // Create a temporary canvas for the current page slice
            const pageCanvas = document.createElement("canvas");
            pageCanvas.width = canvas.width;
            pageCanvas.height = Math.min(pageHeightPx, canvas.height - yPos);

            const ctx = pageCanvas.getContext("2d");

            // Draw the relevant slice of the main canvas onto the page canvas
            ctx.drawImage(
                canvas,
                0, yPos,
                canvas.width, pageCanvas.height,
                0, 0,
                canvas.width, pageCanvas.height
            );

            // Add the image slice to the PDF
            pdf.addImage(
                pageCanvas.toDataURL("image/png"),
                "PNG",
                padding,
                padding,
                imgWidth,
                pageCanvas.height * scale
            );

            yPos += pageCanvas.height;

            // Add a new page if there is more content remaining
            if (yPos < canvas.height) pdf.addPage();
        }

        // Save the generated PDF file
        pdf.save("events-overview.pdf");
    } catch (err) {
        console.error("PDF generation failed:", err);
    } finally {
        // Remove exporting class regardless of success or failure
        document.body.classList.remove("exporting");
    }
};


export default handleExportPDF;