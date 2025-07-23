import { useMutation, useQuery, useQueryClient, type UseQueryResult } from '@tanstack/react-query';

enum ConnectResponseType {
  SessionRestored,
  SessionCreated,
  TooManySessions,
}

type ConnectResponse = {
  session_status: ConnectResponseType;
};

const queryKey = ['session'];

export function useConnect(session_uuid: string): UseQueryResult<ConnectResponse> {
  return useQuery({
    queryKey: [...queryKey, 'connect'],
    queryFn: async () =>
      (
        await fetch('/api/connect', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ session_uuid }),
        })
      ).json() as Promise<ConnectResponse>,
  });
}

type SetNameRequest = {
  session_uuid: string;
  name: string;
};

export function useSetName() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (request: SetNameRequest) =>
      await fetch('/api/set_name', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: [...queryKey, 'list'] }),
  });
}

type Session = {
  uuid: string;
  name: string;
};

type ListResponse = {
  sessions: Session[];
};

export function useList(): UseQueryResult<ListResponse> {
  return useQuery({
    queryKey: [...queryKey, 'list'],
    queryFn: async () => (await fetch('/api/sessions')).json() as Promise<ListResponse>,
  });
}
