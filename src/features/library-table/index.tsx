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
    date_added: new Date(2025, 11, 8),
    date_modified: new Date(2025, 11, 8),
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
    date_added: new Date(2025, 11, 8),
    date_modified: new Date(2025, 11, 8),
    date_published: new Date(2013, 1, 8)
  }
]
