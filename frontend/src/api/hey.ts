import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

const queryKey = ['theOnlyApi'];

export function useGet() {
  return useQuery({
    queryKey,
    queryFn: async () => (await fetch('/api/hey')).json(),
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
