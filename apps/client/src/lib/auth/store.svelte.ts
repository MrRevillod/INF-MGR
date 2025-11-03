import { browser } from "$app/environment"
import { goto } from "$app/navigation"
import { jwtDecode } from "jwt-decode"

import type { Role, User } from "$lib/users/schemas"

export type SessionTokens = {
	accessToken: string
	refreshToken: string
}

export type TokenClaims = {
	user_id: string
	session_id: string
	role: Role
	exp: number
}

class AuthStore {
	user: User | null = $state(null)
	isLoading: boolean = $state(false)

	isAuthenticated: boolean = $derived(this.user !== null)

	constructor() {
		if (browser) this.init()
	}

	private init(): void {
		const tokens = this.getTokensFromStorage()
		if (tokens && this.isTokenValid(tokens.accessToken)) {
			// Los tokens son válidos, se cargarán cuando se llame a /auth/me
		} else {
			this.clearStorage()
		}
	}

	private getTokensFromStorage(): SessionTokens | null {
		const access = localStorage.getItem("ACCESS")
		const refresh = localStorage.getItem("REFRESH")

		return access && refresh ? { accessToken: access, refreshToken: refresh } : null
	}

	private getTokensFromCookies(): SessionTokens | null {
		const getCookie = (name: string): string | undefined => {
			const value = `; ${document.cookie}`
			const parts = value.split(`; ${name}=`)
			return parts.length === 2 ? parts.pop()?.split(";").shift() : undefined
		}

		const access = getCookie("ACCESS")
		const refresh = getCookie("REFRESH")

		return access && refresh ? { accessToken: access, refreshToken: refresh } : null
	}

	private isTokenValid(token: string): boolean {
		try {
			const claims = jwtDecode<TokenClaims>(token)
			const now = Math.floor(Date.now() / 1000)
			return claims.exp > now
		} catch {
			return false
		}
	}

	public getTokens(): SessionTokens | null {
		return this.getTokensFromStorage()
	}

	public setTokens(tokens: SessionTokens): void {
		localStorage.setItem("ACCESS", tokens.accessToken)
		localStorage.setItem("REFRESH", tokens.refreshToken)
	}

	public decodeToken(token: string): TokenClaims | null {
		try {
			return jwtDecode<TokenClaims>(token)
		} catch {
			return null
		}
	}

	public setUser(user: User): void {
		this.user = user
	}

	public hasRole(role: Role): boolean {
		return this.user?.role === role
	}

	public hasAnyRole(roles: Role[]): boolean {
		return this.user ? roles.includes(this.user.role) : false
	}

	public processOAuthCallback(): boolean {
		const tokens = this.getTokensFromCookies()

		if (!tokens || !this.isTokenValid(tokens.accessToken)) {
			return false
		}

		this.setTokens(tokens)
		this.clearCookies()

		return true
	}

	public navigateToDashboard(): void {
		if (!this.user) {
			goto("/auth/login")
			return
		}

		const routes: Record<Role, string> = {
			administrator: "/admin",
			teacher: "/teacher",
			secretary: "/secretary",
			student: "/student",
		}

		goto(routes[this.user.role] ?? "/")
	}

	private clearStorage(): void {
		localStorage.removeItem("ACCESS")
		localStorage.removeItem("REFRESH")
	}

	private clearCookies(): void {
		document.cookie = "ACCESS=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"
		document.cookie = "REFRESH=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;"
	}

	public reset(): void {
		this.user = null
		this.isLoading = false
		this.clearStorage()
		this.clearCookies()
	}

	public logout(): void {
		this.reset()
		goto("/auth/login")
	}
}

export const auth = new AuthStore()
