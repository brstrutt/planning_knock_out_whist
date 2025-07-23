import { useQuery } from '@tanstack/react-query';

export function usePost(session_uuid: string) {
  return useQuery({
    queryKey: ['session'],
    queryFn: async () =>
      (
        await fetch('/api/connect', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ session_uuid }),
        })
      ).json(),
  });
}
