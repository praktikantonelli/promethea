import NoDatabaseDialog from '@/features/no-database-dialog';
import { LibraryTable } from '@/features/library-table';
import AddBookButton from '@/features/add-book-button';

export function HomePage() {

  return (
    <div className="flex h-screen">
      <div className="m-auto text-center space-y-3">
        <div className="space-y-3">
          <AddBookButton />
          <LibraryTable />
        </div>
        <NoDatabaseDialog />
      </div>
    </div>
  )
}

// Necessary for react router to lazy load.
export const Component = HomePage
