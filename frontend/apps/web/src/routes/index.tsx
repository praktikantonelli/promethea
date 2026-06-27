import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@workspace/ui'

export const Route = createFileRoute('/')({
  component: Index,
})

function Index() {
  return (
    <div className="p-2">
      <h3>Welcome Home!</h3>
      <Button onClick={testBackend}>Test backend</Button>
    </div>
  )
}

function testBackend() {
  console.log("asdf");
}
