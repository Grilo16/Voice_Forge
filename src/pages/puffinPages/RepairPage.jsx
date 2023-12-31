import { useSelector } from "react-redux";
import { selectRepairArgs } from "../../features/reducers/repairReducer";
import { StringBuilder } from "../../components/flagInput/StringBuilder";

export const RepairPage = () => {
    
    const launchArgs = useSelector(selectRepairArgs)
    const stateName = "repair"
    const rustCommand = "run_tmux_command"
    const pageName = "Repair"
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






    // const StyledLabel = styled.label`
    // display: grid;
    // grid-template-columns: 1fr 1fr;
    // justify-content: stretch;
    // `
     // const [speakerId, setSpeakerId] = useState({flag: "-i", value: false})
    // const [versionNumber, setVersionNumber] = useState({flag: "-n", value: false})
    // const [date, setDate] = useState({flag: "-d", value: false})
    // const [donorId, setDonorId] = useState({flag: "--donorid", value: false})
    
    // const [vb, setVb] = useState({flag: "--vb", value: false})
    // const [preProc, setPreProc] = useState({flag: "--preproc", value: false})
    // const [donorPreProc, setDonorPreProc] = useState({flag: "--donorpreproc", value: false})

    // const [donorDate, setDonorDate] = useState({flag: "--donordate", value: false})
    // const [donorRvb, setDonorRvb] = useState({flag: "--donorrvb", value: false})
    
    
    // const [outputString, setOutputString] = useState("python3 run.py")
    
    {/*             
            <FlagInput {...repairArgs.i} setOutput={setOutputObj} />
            <FlagInput {...repairArgs.n} setOutput={setOutputObj} />
            <FlagInput {...repairArgs.vb} setOutput={setOutputObj} />
            <FlagInput {...repairArgs.donordate} setOutput={setOutputObj} />
            <FlagInput {...repairArgs.d} setOutput={setOutputObj}/>
                <StyledLabel> -i 
                <input onChange={e => setSpeakerId(current => ({...current, value: e.target.value}))} type="number" placeholder={speakerId.value}/>
                </StyledLabel>
            
                <StyledLabel htmlFor="n"> -n 
                <input onChange={e => setVersionNumber(current => ({...current, value: e.target.value}))} type="number" id="n" placeholder={versionNumber.value}/>
                </StyledLabel>
            
                <StyledLabel htmlFor="d"> -d 
                <input onChange={e => setDate(current => ({...current, value: e.target.value}))} type="week" id="d" placeholder={date.value}/>
                </StyledLabel>
            
                <StyledLabel htmlFor="donorId"> --donorId 
                <input onChange={e => setDonorId(current => ({...current, value: e.target.value}))} type="number" id="donorId" placeholder={donorId.value}/>
                </StyledLabel>

            
            
                <StyledLabel htmlFor="vb"> --vb 
                <input onChange={()=>setVb(current => ({...current, value: !current.value}))} type="checkbox" id="vb"/>
                </StyledLabel>
            
                <StyledLabel htmlFor="preproc"> --preproc 
                <input onChange={e => setPreProc(current => ({...current, value: e.target.value}))} type="text" id="preproc" placeholder={preProc.value}/>
                </StyledLabel>
            
                <StyledLabel htmlFor="donorpreproc"> --donorpreproc 
                <input onChange={e => setDonorPreProc(current => ({...current, value: e.target.value}))} type="text" id="donorpreproc" placeholder={donorPreProc.value}/>
                </StyledLabel>

            
                <StyledLabel htmlFor="donorDate"> --donorDate 
                <input onChange={e => setDonorDate(current => ({...current, value: e.target.value}))} type="text" id="donorDate" placeholder={donorDate.value} disabled={donorRvb.value}/>
                </StyledLabel>
            
                <StyledLabel htmlFor="donorrvb"> --donorrvb 
                <input onChange={e => setDonorRvb(current => ({...current, value: !current.value}))} type="checkbox" id="donorrvb"/>
                </StyledLabel> */}


                
{/* 
        <h2>output : </h2>
        <h1> 
            python3 run.py 
            {speakerId.value ? ` ${speakerId.flag} ${speakerId.value} `:  " "}
            {versionNumber.value ? ` ${versionNumber.flag} ${versionNumber.value} `:  " "}
            {date.value ? ` ${date.flag} ${date.value.replace("W", "")}`:  " "}
            {donorId.value ? ` ${donorId.flag} ${donorId.value} `:  " "}
            {vb.value ? ` ${vb.flag}`:  " "}
            {preProc.value ? ` ${preProc.flag} ${preProc.value} `:  " "}
            {donorPreProc.value ? ` ${donorPreProc.flag} ${donorPreProc.value} `:  " "}
            {donorDate.value && !donorRvb.value ? ` ${donorDate.flag} ${donorDate.value} `:  " "}
            {donorRvb.value ? ` ${donorRvb.flag} `:  " "}
        </h1> */}
{/* {false ? 
<>
<TaskSelectorDiv> 
        <h3>Build</h3>
    </TaskSelectorDiv>
    <TaskSelectorDiv> 
        <h3>Blend</h3>
    </TaskSelectorDiv>
    repair, build, blend
    python3 run.py 
    -i SPEAKERID 
    -n 09 
    -d 2023-14 
    --donorid DONORID 
    [--vb] 
    [--preproc PREPROC]
    [--donorpreproc DONORPREPROC]
    (--donordate DONORDATE ?? --donorvb) 
    <h1>puffin page</h1>
    <h2>Output <b/>: {outputString}</h2>
</>
: null} */}