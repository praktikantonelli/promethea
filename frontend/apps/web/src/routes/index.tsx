import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@workspace/ui'
import { useQuery } from '@tanstack/react-query'

export const Route = createFileRoute('/')({
  component: Index,
})

function Index() {
  const dummyQuery = useQuery({
    queryKey: [],
    queryFn: async () => {
      const response = await fetch('/api/hello')

      if (!response.ok) {
        throw new Error(`Request failed: ${response.status}`)
      }

      return response.text()
    },
    enabled: false,
  })

  return (
    <div className="p-2">
      <h3>Welcome Home!</h3>

      <Button onClick={() => dummyQuery.refetch()}>
        Test backend
      </Button>

      {dummyQuery.isFetching && <p>loading...</p>}

      {dummyQuery.isError && (
        <pre>{String(dummyQuery.error)}</pre>
      )}

      {dummyQuery.data && (
        <pre>{dummyQuery.data}</pre>
      )}
    </div>
  )
}
