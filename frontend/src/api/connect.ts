import { useQuery, type UseQueryResult } from '@tanstack/react-query';

enum ResponseType {
  SessionRestored,
  SessionCreated,
  TooManySessions,
}

type Response = {
  session_status: ResponseType;
};

const queryKey = ['session'];

export function usePost(session_uuid: string): UseQueryResult<Response> {
  return useQuery({
    queryKey,
    queryFn: async () =>
      (
        await fetch('/api/connect', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ session_uuid }),
        })
      ).json() as Promise<Response>,
  });
}
