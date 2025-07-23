import type { JSX } from "react";
import * as api from './api';

export function Sessions(): JSX.Element {
    const allSessions = api.session.useList();

    return <>
        {
            allSessions.data?.sessions.map(
                session => <div key={session.uuid}>Session: {session.name}</div>
            )
        }
    </>;
}
