import {
	Table as FlowBiteTable,
	TableBody,
	TableCell,
	TableHead,
	TableHeadCell,
	TableRow,
} from "flowbite-react"

export interface Column<T = any> {
	name: string
	dataKey: keyof T
	formatter?: (value: any) => React.ReactNode
}

interface TableProps<T> {
	columns: Column<T>[]
	data: T[]
}

export const Table = <T,>({ columns, data }: TableProps<T>) => {
	return (
		<div className="overflow-x-auto">
			<FlowBiteTable striped>
				<TableHead>
					<TableRow>
						{columns.map(column => (
							<TableHeadCell key={column.dataKey.toString()}>
								{column.name}
							</TableHeadCell>
						))}
					</TableRow>
				</TableHead>
				<TableBody className="divide-y">
					{data.map((item: T, index) => (
						<TableRow key={index} className="bg-white">
							{columns.map(column => (
								<TableCell
									key={column.dataKey.toString()}
									className="whitespace-nowrap font-medium text-gray-900"
								>
									{column.formatter
										? column.formatter(item[column.dataKey])
										: String(item[column.dataKey])}
								</TableCell>
							))}
						</TableRow>
					))}
				</TableBody>
			</FlowBiteTable>
		</div>
	)
}
