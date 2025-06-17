/**
 * Converts an array of objects (dictionaries) into a CSV file and triggers a download in the browser.
 *
 * @param {Object[]} data - Array of objects representing the CSV rows.
 * @param {string} filename - The desired filename for the downloaded CSV file.
 */
const handleDownloadCSV = async (data, filename) => {
    try {
        // Extract column headers from the first object in the array
        const headers = Object.keys(data[0]);

        /**
         * Escapes special characters in a CSV field value.
         * Wraps the value in double quotes if it contains quotes, commas, or newlines.
         * Doubles any internal double quotes per CSV format requirements.
         *
         * @param {*} value - The value to escape.
         * @returns {string} The escaped CSV-compatible string.
         */
        const escapeValue = (value) => {
            if (value === null || value === undefined) return '';
            const str = String(value);
            if (str.includes('"') || str.includes(',') || str.includes('\n')) {
                return `"${str.replace(/"/g, '""')}"`;
            }
            return str;
        };

        // Convert each row object to a CSV string using the header order
        const csvRows = data.map(row =>
            headers.map(header => escapeValue(row[header])).join(',')
        );

        // Combine headers and rows into the final CSV string
        const csvContent = [headers.join(','), ...csvRows].join('\n');

        // Create a Blob from the CSV content
        const blob = new Blob([csvContent], { type: 'text/csv' });

        // Generate a temporary download link and trigger the file download
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        a.style.display = 'none';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url); // Clean up the URL object
    } catch (err) {
        console.error("CSV generation failed:", err);
    }
};


export default handleDownloadCSV;
