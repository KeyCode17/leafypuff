import { z } from "zod"

const MINIMUM_PASSWORD_LENGTH = 12
const CODE_LENGTH = 6

export const signInSchema = z.object({
	email: z.string().trim().email("Enter the address you registered with"),
	password: z
		.string()
		.min(
			MINIMUM_PASSWORD_LENGTH,
			`At least ${MINIMUM_PASSWORD_LENGTH} characters`,
		),
})

export const verifyCodeSchema = z.object({
	code: z
		.string()
		.length(CODE_LENGTH, `The code is ${CODE_LENGTH} digits`)
		.regex(/^\d+$/, "Digits only"),
})

export type TSignInForm = z.infer<typeof signInSchema>
export type TVerifyCodeForm = z.infer<typeof verifyCodeSchema>
