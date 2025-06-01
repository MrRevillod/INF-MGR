import ReactDOM from "react-dom/client"
import App from "./app/router"

import { ThemeConfig } from "flowbite-react"
import { BrowserRouter } from "react-router-dom"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"

const rootElement = document.getElementById("root")!
const root = ReactDOM.createRoot(rootElement)

import "./main.css"

const queryClient = new QueryClient()

root.render(
	<BrowserRouter>
		<QueryClientProvider client={queryClient}>
			<ThemeConfig dark={false} />
			<App />
		</QueryClientProvider>
	</BrowserRouter>,
)
