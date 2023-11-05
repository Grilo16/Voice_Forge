import { useState } from "react";
import styled from "styled-components";

export const PuffinPage = () => {


    const repairFlags = {
        required: {
            i: "speakerId",
            n: "VersionNumber",
            d: "Date YYYY-WW",
            donorId: "DONORID", 
        },

        optional: {
            vb: "",
            preproc: "PREPROC",
            donorpreproc: "DONORPREPROC",
        },

        switch: {
            1: {
                donorDate: "DONORDATE",
                donorrvb: "", 
            }
        }
        
    }


    return (
           <div>
            <h1>Puffin Voices </h1>
           </div>

    )
};

