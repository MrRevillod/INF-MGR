class ApplicationStore {
	pageTitle: string = $state("")
	currentRoute: string = $state("")

	setTitle(title: string) {
		this.pageTitle = title
	}

	setRoute(route: string) {
		this.currentRoute = route
	}

	clear() {
		this.pageTitle = ""
		this.currentRoute = ""
	}
}

export const appStore = new ApplicationStore()
