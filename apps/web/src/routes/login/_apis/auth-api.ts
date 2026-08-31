import { request, requestWithoutData } from "#/lib/api-client"

export type TChallengeResponse = {
	expires_in: number
}

export type TSessionResponse = {
	access_token: string
	refresh_token: string
	token_type: string
	expires_in: number
}

export type TSignInRequest = {
	email: string
	password: string
}

export type TVerifyCodeRequest = {
	email: string
	code: string
	device_id: string
}

export const postSignIn = (
	payload: TSignInRequest,
): Promise<TChallengeResponse> =>
	request<TChallengeResponse>({
		path: "/v1/auth/sign-in",
		method: "POST",
		body: payload,
	})

export const postVerifySignIn = (
	payload: TVerifyCodeRequest,
): Promise<TSessionResponse> =>
	request<TSessionResponse>({
		path: "/v1/auth/sign-in/verify",
		method: "POST",
		body: payload,
	})

export const postVerifyEmail = (payload: {
	email: string
	code: string
}): Promise<void> =>
	requestWithoutData({
		path: "/v1/auth/verify-email",
		method: "POST",
		body: payload,
	})
