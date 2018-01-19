import msgpack from "./msgpack";

/*lots of console.log() statements for educational purposes in this file, don't forget to remove them in production*/

function convertToBinary(socket){
  let parentOnConnOpen = socket.onConnOpen;

  socket.onConnOpen = function(){
    //setting this to arraybuffer will help us not having to deal with blobs
    this.conn.binaryType = 'arraybuffer';
    parentOnConnOpen.apply(this, arguments);
  };

  //we also need to override the onConnMessage function, where we'll be checking
  //for binary data, and delegate to the default implementation if it's not what we expected
  let parentOnConnMessage = socket.onConnMessage;

  socket.onConnMessage = function (rawMessage){
    if(!(rawMessage.data instanceof window.ArrayBuffer)){
      return parentOnConnMessage.apply(this, arguments);
    }
    let msg = decodeMessage(rawMessage.data);
    let topic = msg.topic;
    let event = msg.event;
    let payload = msg.payload;
    let ref = msg.ref;

    this.log("receive", (payload.status || "") + " " + topic + " " + event + " " + (ref && "(" + ref + ")" || ""), payload);

    // The default implementation of onConnMessage does this to reset the heartbeat timeout.
    // Duplicate this because we are never calling the default implementation, for now.
    if(ref && ref === this.pendingHeartbeatRef){ this.pendingHeartbeatRef = null; }

    this.channels.filter(function (channel) {
      return channel.isMember(topic);
    }).forEach(function (channel) {
      return channel.trigger(event, payload, ref);
    });
    this.stateChangeCallbacks.message.forEach(function (callback) {
      return callback(msg);
    });
  }

  return socket;
}

function decodeMessage (rawdata) {
    if(!rawdata){
        return undefined;
    }
    let binary = new Uint8Array(rawdata);
    let data;
    data = binary;

    let msg = msgpack.decode(data);
    return msg;
}

export default {
  convertToBinary
};
