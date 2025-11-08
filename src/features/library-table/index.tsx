// see https://ui.shadcn.com/docs/components/data-table
"use client";

import * as React from "react";
import {
  ColumnDef,
  ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
  VisibilityState,
} from "@tanstack/react-table";

import {
  ArrowUpDown,
  ChevronDown,
  MoreHorizontal
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

export type SeriesAndVolume = {
  series: string
  sort: string
  volume: number
}

export type BookRecord = {
  book_id: number
  title: string
  sort: string
  authors: string[]
  authors_sort: string[]
  series_and_volume: SeriesAndVolume[]
  number_of_pages: number
  goodreads_id: number
  date_added: Date
  date_published: Date
  date_modified: Date
}

export const book_records: BookRecord[] = [
  {
    book_id: 1,
    title: "The Fellowship of the Ring",
    sort: "Fellowship of the Ring, The",
    authors: ["J. R. R. Tolkien"],
    authors_sort: ["Tolkien, J. R. R."],
    series_and_volume: [
      {
        series: "Middle-Earth",
        sort: "Middle-Earth",
        volume: 1
      },
      {
        series: "The Lord of the Rings",
        sort: "Lord of the Rings, The",
        volume: 1
      }
    ],
    number_of_pages: 432,
    goodreads_id: 61215351,
    date_added: new Date(2025, 10, 8),
    date_modified: new Date(2025, 10, 8),
    date_published: new Date(1954, 7, 29)
  },
  {
    book_id: 2,
    title: "A Memory of Light",
    sort: "Memory of Light, A",
    authors: ["Robert Jordan", "Brandon Sanderson"],
    authors_sort: ["Jordan, Robert", "Sanderson, Brandon"],
    series_and_volume: [
      {
        series: "Wheel of Time",
        sort: "Wheel of Time",
        volume: 14
      },
    ],
    number_of_pages: 912,
    goodreads_id: 7743175,
    date_added: new Date(2025, 10, 8),
    date_modified: new Date(2025, 10, 8),
    date_published: new Date(2013, 1, 8)
  }
]

export const columns: ColumnDef<BookRecord>[] = [
  {
    accessorKey: "book_id",
    header: "Book ID",
  },
  {
    accessorKey: "title",
    header: ({ column }) => {
      return (
        <Button variant="ghost" onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}>
          Title
          <ArrowUpDown />
        </Button>
      )
    }
  },
  {
    accessorKey: "sort",
    header: "Title Sort",
  },
  {
    accessorKey: "authors",
    header: "Authors",
    cell: ({ row }) => {
      const authors = row.getValue("authors");
      let formatted: string | null;
      if (authors instanceof Array) {
        formatted = authors.join(", ")
      } else {
        formatted = null;
      }
      return <div>{formatted}</div>
    }
  },
  {
    accessorKey: "authors_sort",
    header: "Authors Sort",
    cell: ({ row }) => {
      const authors = row.getValue("authors_sort");
      let formatted: string | null;
      if (authors instanceof Array) {
        formatted = authors.join(", ")
      } else {
        formatted = null;
      }
      return <div>{formatted}</div>
    }
  },
  {
    accessorKey: "series_and_volume",
    header: "Series and Volume",
    cell: ({ row }) => {
      const series_and_volume: SeriesAndVolume[] = row.getValue("series_and_volume");
      let formatted = series_and_volume.map((element) => `${element.series} #${element.volume}`).join(", ");
      return <div>{formatted}</div>
    }
  },
  {
    accessorKey: "number_of_pages",
    header: "No. Pages",
  },
  {
    accessorKey: "goodreads_id",
    header: "Goodreads ID",
  },
  {
    accessorKey: "date_added",
    header: "Date Added",
    cell: ({ row }) => {
      const value = row.getValue("date_added");
      let formatted: string | null;
      if (value instanceof Date) {
        formatted = value.toLocaleDateString()
      } else {
        formatted = null
      }

      return <div>{formatted}</div>
    }
  },
  {
    accessorKey: "date_modified",
    header: "Date Modified",
    cell: ({ row }) => {
      const value = row.getValue("date_modified");
      let formatted: string | null;
      if (value instanceof Date) {
        formatted = value.toLocaleDateString()
      } else {
        formatted = null
      }

      return <div>{formatted}</div>
    }
  },
  {
    accessorKey: "date_published",
    header: "Date Pulished",
    cell: ({ row }) => {
      const value = row.getValue("date_published");
      let formatted: string | null;
      if (value instanceof Date) {
        formatted = value.toLocaleDateString()
      } else {
        formatted = null
      }

      return <div>{formatted}</div>
    }
  }
]

export function LibraryTable() {
  const [sorting, setSorting] = React.useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([]);

  const [columnVisibility, setColumnVisibility] = React.useState<VisibilityState>({
    authors_sort: false,
    sort: false
  });
  const [rowSelection, setRowSelection] = React.useState({});

  const table = useReactTable({
    data: book_records,
    columns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
    },
  })

  return (
    <div className="w-full">
      <div className="flex items-center py-4">
        <Input
          placeholder="Filter books..."
          value={(table.getColumn("title")?.getFilterValue() as string) ?? ""}
          onChange={(event) =>
            table.getColumn("title")?.setFilterValue(event.target.value)}
          className="max-w-sm" />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="ml-auto">
              Columns <ChevronDown />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {table.getAllColumns().filter((column) => column.getCanHide()).map((column) => {
              return (
                <DropdownMenuCheckboxItem
                  key={column.id}
                  className="capitalize"
                  checked={column.getIsVisible()}
                  onCheckedChange={(value) =>
                    column.toggleVisibility(!!value)
                  }
                >
                  {typeof column.columnDef.header === "string" ? column.columnDef.header : column.id}
                </DropdownMenuCheckboxItem>
              )
            })}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <div className="overflow-hidden rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  )
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center"
                >
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      <div className="flex items-center justify-end space-x-2 py-4">
        <div className="text-muted-foreground flex-1 text-sm">
          {table.getFilteredSelectedRowModel().rows.length} of{" "}
          {table.getFilteredRowModel().rows.length} row(s) selected.
        </div>
        <div className="space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  )
}
