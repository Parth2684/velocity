import { create } from "zustand";
import { StoreAction, StoreState, TransferType } from "./types";
import { invoke } from "@tauri-apps/api/core";

export const store = create<StoreState & StoreAction>((set, get) => ({
  availableDevices: new Map(),
  send_files: new Map(),
  receive_files: new Map(),
  otp: null,
  connectedTo: null,

  serveAndConnectQuic: async () => {
    try {
      await invoke("serve_and_connect_quic");
    } catch (err) {
      console.error("error serving: " + err);
    }
  },

  scan: async (discovery) => {
    try {
      await invoke("scan", {
        discovery,
      });
    } catch (err) {
      console.error("error scanning: " + err);
    }
  },
  receiveCertAndConnectQuic: async (txt_properties, otp) => {
    try {
      await invoke("receive_cert_and_connect_quic", {
        txt_properties,
        otp,
      });
    } catch (err) {
      console.error("error connecting device: " + err);
    }
  },
  addAvailableDevice: (data) => {
    set((state) => ({
      availableDevices: new Map(state.availableDevices).set(
        data.fullname,
        data,
      ),
    }));
  },
  removeAvailableDevice: (name) => {
    const beforeDelete = new Map(get().availableDevices);
    beforeDelete.delete(name);
    set({ availableDevices: beforeDelete });
  },
  setOtp: (otp) => {
    set({ otp });
  },
  addConnectedTo: (deviceName) => {
    invoke("receive_file")
    set({ connectedTo: deviceName });
  },
  removeConnectedTo: () => {
    set({ connectedTo: null });
  },
  addSendTransfer: (transfers) => {
    const existingFiles = new Map(get().send_files);
    transfers.forEach((data, entry) => {
      existingFiles.set(entry, {
        path: data.path,
        name: data.name,
        data_type: data.data_type,
        file_size: data.file_size,
        transferred: 0,
        progress: 0,
        speed: 0,
        completedIn: null,
        startedAt: null,
        cancelled: false
      });
    });
    set({ send_files: existingFiles });
  },
  addReceiveTransfer: (transfers) => {
    const existingFiles = new Map(get().send_files);
    transfers.forEach((data, entry) => {
      existingFiles.set(entry, {
        path: data.path,
        name: data.name,
        data_type: data.data_type,
        file_size: data.file_size,
        transferred: 0,
        progress: 0,
        speed: 0,
        completedIn: null,
        startedAt: null,
        cancelled: false
      });
    });
    set({ send_files: existingFiles });
  },
  updateProgress: (progress, transferType) => {
    if (transferType === TransferType.Send) {
      const sendFiles = new Map(get().send_files);
      const transferFile = sendFiles.get(progress.path);
      if (transferFile === undefined) return;
      else {
        const updatedFile = {
          ...transferFile,
          progress: progress.progress,
          speed: progress.speed,
          transferred: progress.transferred,
          startedAt: transferFile.startedAt ?? Date.now()
        };
        sendFiles.set(progress.path, updatedFile);
        set({ send_files: sendFiles });
      }
    }
    else {
      const receiveFiles = new Map(get().receive_files);
      const receiveFile = receiveFiles.get(progress.path);
      if (receiveFile === undefined) return;
      else {
        const updatedFile = {
          ...receiveFile,
          progress: progress.progress,
          speed: progress.speed,
          transferred: progress.transferred,
          startedAt: receiveFile.startedAt ?? Date.now()
        }
        receiveFiles.set(progress.path, updatedFile);
        set({ receive_files: receiveFiles })
      }
    }
  },
  checkCompleted: (path, completedIn, transferFile) => {
    if (transferFile === TransferType.Send) {
      const sendfiles = new Map(get().send_files);
      const sentFile = sendfiles.get(path)
      if(sentFile === undefined) return
      const updated = {
        ...sentFile,
        completedIn
      }
      sendfiles.set(path, updated)
      set({ send_files: sendfiles })
    }
    else {
      const receiveFiles = new Map(get().receive_files)
      const receivedFile = receiveFiles.get(path)
      if (receivedFile === undefined) return
      const updated = { 
        ...receivedFile,
        completedIn
      }
      receiveFiles.set(path, updated)
      set({ receive_files: receiveFiles })
    }
  },
  cancelSend: async(path) => {
    try {
      await invoke("cancel_transfer_file", {
        path
      })
      const sendingFiles = new Map(get().send_files)
      sendingFiles.delete(path)
      set({ send_files: sendingFiles })
    } catch (err) {
      console.error("Error cancelling transfer: " + err )
    }
  },
  cancelReceive: (path) => {
    const receivingFiles = new Map(get().receive_files)
    receivingFiles.delete(path)
    set({ receive_files: receivingFiles })
  },
  send: async (paths) => {
    try {
      await invoke("send_file", {
        paths
      })
    } catch (err) {
      console.error("error sending files: " + err)
    }
  }
}));
