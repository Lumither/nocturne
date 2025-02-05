import React from 'react';


export abstract class Activity {
    protected constructor(
        readonly time: Date
    ) {
    }

    abstract toElement(): React.ReactElement;

}

export abstract class GhActivity extends Activity {
    protected constructor(time: Date) {
        super(time);
    }
}
