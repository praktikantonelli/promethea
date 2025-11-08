import BuiltWith from '@/features/built-with';
import NoDatabaseDialog from '@/features/no-database-dialog';
import { LibraryTable } from '@/features/library-table';

export function HomePage() {

  return (
    <div className="flex h-screen">
      <div className="m-auto text-center space-y-3">
        <div className="space-y-3">
          <LibraryTable />
        </div>
        <NoDatabaseDialog />
      </div>
    </div>
  )
}

// Necessary for react router to lazy load.
export const Component = HomePage
