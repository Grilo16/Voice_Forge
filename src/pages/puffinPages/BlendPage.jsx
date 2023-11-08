import { useSelector } from "react-redux";
import { selectBuildArgs } from "../../features/reducers/buildReducer";
import { StringBuilder } from "../../components/flagInput/StringBuilder";

export const BlendPage = () => {

    const launchArgs = useSelector(selectBuildArgs)
    const stateName = "blend"
    const rustCommand = "run_tmux_command"
    const pageName = "Blend"
    const flagData = {
        i: {
            label: "Speaker Id",
            flag: "-i",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        d: {
            label: "Date (YYYY-WW)",
            flag: "-d",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        n: {
            label: "Starting Version",
            flag: "-n",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        vb: {
            label: "vb",
            flag: "--vb",
            type: "checkbox",
            required: false,
            altFlags : [],
        }, 
        accent: {
            label: "Accent",
            flag: "--accent",
            type: "text",
            required: false,
            altFlags : [],
        }, 
        prepoc: {
            label: "Preprocessing",
            flag: "--preproc",
            type: "dropdown",
            options: ["rnnoise", "postfish", "voicefixer", "rnnoise_postfish"],
            required: false,
            altFlags : [],
        }, 
        donorid: {
            label: "Donor Id",
            flag: "--donorid",
            type: "text",
            required: true,
            altFlags : [],
        }, 
        donordate: {
            label: "Donor Date",
            flag: "--donordate",
            type: "text",
            required: false,
            altFlags : [{
                label: "Donor Date",
                altLabel: "donor vb",
                flag: "--donorvb",
                type: "checkbox",
                required: false,
            },],
        }, 
        donorprepoc: {
            label: "Donor Preprocessing",
            flag: "--donorpreproc",
            type: "dropdown",
            options: ["rnnoise", "postfish", "voicefixer", "rnnoise_postfish"],
            required: false,
            altFlags : [],
        }, 
        sada: {
            label: "Dry run",
            flag: "--dry",
            type: "checkbox",
            required: true,
            altFlags : [],
        }, 
        doaasdanorid: {
            label: "Super dry run",
            flag: "--superdry",
            type: "checkbox",
            required: true,
            altFlags : [],
        }, 
    }
    
    return (
        <StringBuilder launchArgs={launchArgs} stateName={stateName} rustCommand={rustCommand} pageName={pageName} flagData={flagData}/>
    )
}
