import { Column, Table } from "@/shared/components/Table"
import { api } from "@/shared/lib/axios"
import { useQuery } from "@tanstack/react-query"
import React from "react"

import dayjs from "dayjs"

interface User {
	id: string
	username: string
	email: string
	createdAt: string
	updatedAt: string
}

const Home: React.FC = () => {
	const { data } = useQuery<User[]>({
		queryKey: ["users"],
		queryFn: async () => {
			return await api.get("/users").then(res => res.data.data)
		},
	})

	const dateFormatter = (value: any) => {
		return dayjs(value?.toString()).format("YYYY-MM-DD") ?? "N/A"
	}

	const columns: Column<User>[] = [
		{ name: "ID", dataKey: "id" },
		{ name: "Username", dataKey: "username" },
		{ name: "Email", dataKey: "email" },
		{
			name: "Created At",
			dataKey: "createdAt",
			formatter: dateFormatter,
		},
		{
			name: "Updated At",
			dataKey: "updatedAt",
			formatter: dateFormatter,
		},
	]

	return (
		<>
			<Table data={data ?? []} columns={columns} />
		</>
	)
}

export default Home
