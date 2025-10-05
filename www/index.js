import init, { parse_url_params, start_redirect } from './pkg/tenkyo.js';

async function run() {
    try {
        // Initialize WASM module
        await init();

        // Parse URL parameters
        const config = parse_url_params();

        // Start the redirect process
        start_redirect(config);
    } catch (error) {
        console.error('Error loading WASM module:', error);

        // Fallback: show error message and redirect using JavaScript
        const messageEl = document.getElementById('message');
        if (messageEl) {
            messageEl.textContent = 'Error loading. Redirecting via fallback...';
        }

        // Parse URL parameters manually as fallback
        const params = new URLSearchParams(window.location.search);
        const url = params.get('url') || 'https://blog.poyea.me';
        const delay = parseInt(params.get('delay') || '3', 10);

        // Redirect after delay
        setTimeout(() => {
            window.location.href = url;
        }, delay * 1000);
    }
}

run();
