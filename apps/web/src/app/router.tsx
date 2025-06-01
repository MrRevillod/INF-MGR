import React from "react"

import { Route, Routes } from "react-router-dom"

import Home from "./home"

const App: React.FC = () => {
	return (
		<Routes>
			<Route path="/home" element={<Home />} />
		</Routes>
	)
}

export default App
