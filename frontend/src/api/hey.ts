import { useMutation, useQueryClient, useSuspenseQuery } from '@tanstack/react-query';

type Response = {
  text: string;
};

const queryKey = ['hey'];

export function useGet(): Response {
  return useSuspenseQuery({
    queryKey,
    queryFn: async () => (await fetch('/api/hey')).json() as Promise<Response>,
  }).data;
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
