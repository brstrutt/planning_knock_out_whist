import { useMutation, useQuery, useQueryClient, type UseQueryResult } from '@tanstack/react-query';

type Response = {
  text: string;
};

const queryKey = ['hey'];

export function useGet(): UseQueryResult<Response> {
  return useQuery({
    queryKey,
    queryFn: async () => (await fetch('/api/hey')).json() as Promise<Response>,
  });
}

export function usePost() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (newText: string) =>
      await fetch('/api/hey', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ text: newText }),
      }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey }),
  });
}
