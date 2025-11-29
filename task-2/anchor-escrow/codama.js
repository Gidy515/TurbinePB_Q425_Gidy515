import { createCodamaConfig } from "gill";

export default createCodamaConfig({
  idl: "target/idl/anchor_escrow.json",
  clientJs: "clients/js/src/generated",
});
