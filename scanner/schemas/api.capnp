@0xbc4c98a185cc6038;

using IdType = UInt64;

struct Request {
    id @0 :IdType;

    union {
        quit @1 :Void;

        getPresets @2 :Void;

        renderBp :group {
            bpString @3 :Text;
            preset @4 :Text;
        }
    }
}

struct Response {
    id @0 :IdType;

    union {
        bye @1 :Void;
        requestError @2 :ErrorType;

        presets @3 :List(Text);

        renderedBp :group {
            image @4 :Data;
            missing @5 :List(Text);
        }
    }

    enum ErrorType {
        invalidRequest @0;
        invalidPreset @1;

        queueFull @2;
        processingError @3;
    }
}
