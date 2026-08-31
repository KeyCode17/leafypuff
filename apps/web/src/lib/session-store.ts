import { Store } from "@tanstack/store"

const DEVICE_ID_KEY = "leafypuff.cms.device-id"

type TSession = {
	accessToken: string | null
	refreshToken: string | null
	deviceId: string
}

const readDeviceId = (): string => {
	const stored = window.localStorage.getItem(DEVICE_ID_KEY)
	if (stored) {
		return stored
	}
	const minted = crypto.randomUUID()
	window.localStorage.setItem(DEVICE_ID_KEY, minted)
	return minted
}

export const sessionStore = new Store<TSession>({
	accessToken: null,
	refreshToken: null,
	deviceId: readDeviceId(),
})

export const startSession = (accessToken: string, refreshToken: string): void =>
	sessionStore.setState((state) => ({ ...state, accessToken, refreshToken }))

export const endSession = (): void =>
	sessionStore.setState((state) => ({
		...state,
		accessToken: null,
		refreshToken: null,
	}))

export const isSignedIn = (): boolean => sessionStore.state.accessToken !== null
