/// <reference types="vite/client" />

type TImportMetaEnv = {
	readonly VITE_API_BASE_URL?: string
}

interface ImportMeta {
	readonly env: TImportMetaEnv
}
