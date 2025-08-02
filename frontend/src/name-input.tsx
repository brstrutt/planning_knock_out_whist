import { useCallback, useEffect, useRef, type FormEvent, type JSX } from "react";
import type { User } from "./api/users";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from './api';


export function NameInputField(props: { session_uuid: string, user: User }): JSX.Element {
    const { session_uuid, user } = props;
    const messageInput = useRef<HTMLInputElement>(null);
    useEffect(
        () => {
            if (messageInput.current !== undefined && messageInput.current !== null) {
                messageInput.current.value = user.name;
            }
        },
        // Only trigger this on the first render.
        // eslint-disable-next-line react-hooks/exhaustive-deps
        []
    );

    const queryClient = useQueryClient();
    const setName = useMutation({
        mutationFn: (newName: string) => api.users.update(session_uuid, { ...user, name: newName }),
        onSuccess: () => queryClient.invalidateQueries({ queryKey: ['users'] }),
    });
    const submit = useCallback(
        (e: FormEvent<HTMLFormElement>) => {
            e.preventDefault();
            setName.mutate(messageInput.current?.value ?? 'Error: Name was set but input field could not be found!');
        },
        [setName],
    );

    return <>
        <form
            onSubmit={submit}
        >
            <input id='my-user-name-input' ref={messageInput} />
        </form>
    </>;
}