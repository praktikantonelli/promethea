import { createFileRoute } from "@tanstack/react-router";
import { Button } from "@workspace/ui";
import { useQuery } from "@tanstack/react-query";
import type { MyDummyStruct } from "@workspace/lib";

export const Route = createFileRoute("/")({
  component: Index,
});

async function fetchTestTypes(): Promise<MyDummyStruct> {
  const response = await fetch("/api/test-types");

  if (!response.ok) {
    throw new Error(`Failed to fetch test types: ${response.status}`);
  }

  return response.json() as Promise<MyDummyStruct>;
}

function Index() {
  const query = useQuery({
    queryKey: ["test-types"],
    queryFn: fetchTestTypes,
    enabled: false,
  });

  return (
    <div>
      <Button onClick={() => query.refetch()} disabled={query.isFetching}>
        {query.isFetching ? "Loading..." : "Fetch test types"}
      </Button>

      {query.isError && <div>Error: {query.error.message}</div>}

      {query.data && <pre>{JSON.stringify(query.data, null, 2)}</pre>}
    </div>
  );
}
