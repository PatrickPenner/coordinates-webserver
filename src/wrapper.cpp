#include "wrapper.h"

#include <sstream>

using namespace RDKit;

std::shared_ptr<RWMol> smiles_to_mol(const std::string& smiles) {
    return std::shared_ptr<RWMol>(SmilesToMol(smiles, {}));
}

void generate_conformers(const std::shared_ptr<RWMol>& mol) {
    DGeomHelpers::EmbedMultipleConfs(*mol, 10);
}

rust::String mol_to_sdf(const std::shared_ptr<RWMol>& mol) {
    std::stringstream sd_stream;
    for (unsigned i = 0; i < mol->getNumConformers(); i++) {
        sd_stream << MolToMolBlock(*mol, true, i) <<  "$$$$\n";
    }
    return sd_stream.str();
}

