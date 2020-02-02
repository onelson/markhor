import path from 'path';
import os from 'os';

import { MarkhorDB } from 'markhor-neon';
export const markhorDB = new MarkhorDB(path.resolve(os.homedir(), ".markhordb.sqlite"));

