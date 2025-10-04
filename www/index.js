import init, { parse_url_params, start_redirect } from './pkg/tenkyo.js';

async function run() {
    try {
        // Initialize WASM module
        await init();

        console.log('WASM module loaded successfully');

        // Parse URL parameters
        const config = parse_url_params();

        // Start the redirect process
        start_redirect(config);
    } catch (error) {
        console.error('Error loading WASM module:', error);
        
        // Fallback: show error message
        const messageEl = document.getElementById('message');
        if (messageEl) {
            messageEl.textContent = 'Error loading. Redirecting via fallback...';
        }
        
        // The meta refresh tag will handle the redirect
    }
}

run();
