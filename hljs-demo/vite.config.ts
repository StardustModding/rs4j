import { defineConfig } from "vite";

export default defineConfig({
    server: {
        port: 4000,
        strictPort: true,

        hmr: {
            port: 4000,
            protocol: "wss",
            clientPort: 443,
        },
    },
});
