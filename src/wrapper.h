#include <GraphMol/RWMol.h>
#include <GraphMol/DistGeomHelpers/Embedder.h>
#include <GraphMol/FileParsers/FileParsers.h>
#include <GraphMol/SmilesParse/SmilesParse.h>

#include <rust/cxx.h>

using namespace RDKit;

std::shared_ptr<RWMol> smiles_to_mol(const std::string& smiles);
void generate_conformers(const std::shared_ptr<RWMol>& mol);
rust::String mol_to_sdf(const std::shared_ptr<RWMol>& mol);
