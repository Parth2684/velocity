



export enum Discovery {
  On = "on",
  Off = "off"
}

export enum CustomMatcherType {
  App = "app",
  Archive = "archive",
  Audio = "audio",
  Book = "book",
  Custom = "custom",
  Doc = "doc",
  Font = "font",
  Image = "image",
  Text = "text",
  Video = "video"
}

export interface AvailableDevice {
  ty_domain: string;
  sub_ty_domain: string | null;
  fullname: string;
  host: string;
  port: number;
  txt_properties: Map<string, string>
}

export enum TransferType {
  Send,
  Receive
}

export interface GetMetadata {
  path: string;
  name: string;
  data_type: CustomMatcherType;
  file_size: number;
}

export interface GetProgress {
  path: string,
  transferred: number,
  progress: number,
  speed: number
}


export interface Metadata {
  path: string;
  name: string;
  data_type: CustomMatcherType;
  file_size: number;
  transferred: number;
  progress: number;
  speed: number
  completedIn: number | null
  startedAt: number | null
  cancelled: boolean
}

export type StoreState = {
  availableDevices: Map<string, AvailableDevice>;
  send_files: Map<string, Metadata>
  receive_files: Map<string, Metadata>
  otp: string | null
  connectedTo: string | null
}

export type StoreAction = {
  serveAndConnectQuic: () => Promise<void>;
  scan: (discovery: Discovery) => Promise<void>;
  receiveCertAndConnectQuic: (txt_properties: Map<string, string>, otp: string) => Promise<void>;
  addAvailableDevice: (data: AvailableDevice) => void
  removeAvailableDevice: (name: string) => void
  setOtp: (otp: string) => void
  addConnectedTo: (deviceName: string) => void
  removeConnectedTo: () => void;
  addSendTransfer: (transfers: Map<string, GetMetadata>) => void
  addReceiveTransfer: (transfers: Map<string, GetMetadata>) => void
  updateProgress: (progress: GetProgress, transferType: TransferType) => void
  checkCompleted: (path: string, completedIn: number, transfertype: TransferType) => void
  cancelSend: (path: string) => Promise<void>
  cancelReceive: (path: string) => void
  send: (paths: string[]) => Promise<void>
}