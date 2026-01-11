// see https://ui.shadcn.com/docs/components/data-table
"use client";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import * as React from "react";
import {
  Column,
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
  ArrowUpRight,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
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
import { error as logError } from "@tauri-apps/plugin-log";

export type SeriesAndVolumeRecord = {
  series: string
  sort: string
  volume: number
  goodreads_id: number
}

export type AuthorRecord = {
  name: string
  sort: string
  goodreads_id: number
}

export type BookRecord = {
  book_id: number
  title: string
  sort: string
  authors: AuthorRecord[]
  series_and_volume: SeriesAndVolumeRecord[]
  number_of_pages: number
  goodreads_id: number
  date_added: Date
  date_published: Date
  date_modified: Date
}

function getColumnLabel(column: Column<BookRecord, unknown>) {
  const metaLabel = column.columnDef.meta as string | undefined;
  if (metaLabel) return metaLabel;

  const header = column.columnDef.header;
  if (typeof header === "string") return header;

  return column.id;
}

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
    },
    sortingFn: (rowA, rowB) => {
      const a = rowA.original.sort ?? "";
      const b = rowB.original.sort ?? "";
      return a.localeCompare(b, undefined, { sensitivity: "base" });
    }
  },
  {
    accessorKey: "sort",
    header: "Title Sort",
  },
  {
    accessorKey: "authors",
    header: ({ column }) => {
      return (
        <Button variant="ghost" onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}>
          Authors
          <ArrowUpDown />
        </Button>
      )
    },
    cell: ({ row }) => {
      const authors: AuthorRecord[] = row.getValue("authors");
      let formatted = authors.map((element) => `${element.name}`).join(", ");
      return <div>{formatted}</div>
    },
    sortingFn: (rowA, rowB) => {
      const a = rowA.original.authors[0].sort ?? "";
      const b = rowB.original.authors[0].sort ?? "";
      return a.localeCompare(b, undefined, { sensitivity: "base" });
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
    header: ({ column }) => {
      return (
        <Button variant="ghost" onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}>
          Series and Volume
          <ArrowUpDown />
        </Button>
      )
    },
    cell: ({ row }) => {
      const series_and_volume: SeriesAndVolumeRecord[] = row.getValue("series_and_volume");
      let formatted = series_and_volume.map((element) => `${element.series} #${element.volume}`).join(", ");
      return <div>{formatted}</div>
    },
    meta: "Series and Volume" // needed to correctly format column name in dropdown-menu
  },
  {
    accessorKey: "number_of_pages",
    header: "No. Pages",
  },
  {
    accessorKey: "goodreads_id",
    header: "Goodreads ID",
    cell: ({ row }) => {
      const id: string = row.getValue("goodreads_id");
      const link = `https://goodreads.com/book/show/${id}`;
      return <a href={link} target="_blank" className="inline-flex">{id}<ArrowUpRight size={13} /></a>

    }
  },
  {
    accessorKey: "date_added",
    header: "Date Added",
    cell: ({ row }) => {
      const value = row.getValue("date_added");
      let formatted: string | null;
      if (value instanceof Date) {
        formatted = value.toLocaleDateString()
      } else if (typeof value === "string") {
        let date = new Date(value);
        formatted = date.toLocaleDateString();
      } else {
        formatted = null;
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
      } else if (typeof value === "string") {
        let date = new Date(value);
        formatted = date.toLocaleDateString();
      } else {
        formatted = null;
      }

      return <div>{formatted}</div>
    }
  },
  {
    accessorKey: "date_published",
    header: "Date Published",
    cell: ({ row }) => {
      const value = row.getValue("date_published");
      let formatted: string | null;
      if (value instanceof Date) {
        formatted = value.toLocaleDateString()
      } else if (typeof value === "string") {
        let date = new Date(value);
        formatted = date.toLocaleDateString();
      } else {
        formatted = null
      }

      return <div>{formatted}</div>
    }
  }
]

export function LibraryTable() {
  const [data, setData] = React.useState<BookRecord[]>([]);

  const [_loading, setLoading] = React.useState(true);
  const [_error, setError] = React.useState<unknown>(null);

  React.useEffect(() => {
    let cancelled = false;
    let unlisten: null | (() => void) = null;

    const fetchBooks = async () => {
      try {
        setLoading(true);
        const result = await invoke<BookRecord[]>("fetch_books");
        if (!cancelled) {
          setData(result);
          setError(null);
        }
      } catch (e) {
        logError(`failed to get data for book table: ${e}`);
        if (!cancelled) setError(e);
      } finally {
        if (!cancelled) setLoading(false);
      }
    };

    (async () => {
      // initial table load
      await fetchBooks();

      // listen to DB change event
      unlisten = await listen("db:changed", () => {
        fetchBooks();
      })
    })();

    return () => {
      // cleanup => stop listening for event
      cancelled = true;
      unlisten?.();
    };
  }, []);



  const [sorting, setSorting] = React.useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([]);

  const [columnVisibility, setColumnVisibility] = React.useState<VisibilityState>({
    authors_sort: false,
    sort: false
  });
  const [rowSelection, setRowSelection] = React.useState({});

  const table = useReactTable({
    data,
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
                  {getColumnLabel(column)}
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
