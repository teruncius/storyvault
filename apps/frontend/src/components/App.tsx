import { Button } from "@storyvault/frontend/components/ui/button"
import { useHealth } from "@storyvault/frontend/hooks/health"
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@storyvault/frontend/lib/query-client";

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Content />
    </QueryClientProvider>
  )
}

function Content() {
  const { data } = useHealth();
  return (
    <div className="flex min-h-svh flex-col items-center justify-center">
      <Button>{JSON.stringify(data)}</Button>
    </div>
  );
}