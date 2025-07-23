import type { JSX } from "react";
import * as api from './api';

export function Sessions(): JSX.Element {
    const allSessions = api.session.useList();

    return <div className='Sessions'>
        {
            allSessions.sessions.map(
                session => <Session key={session.uuid} session={session} />
            )
        }
    </div>;
}

function Session(props: { session: api.session.Session }): JSX.Element {
    const { session } = props;
    return (
        <div className='session'>{session.name}</div>
    );
}
