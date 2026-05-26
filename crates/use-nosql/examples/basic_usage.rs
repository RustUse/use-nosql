use use_nosql::{
    CacheKey, CacheNamespace, CollectionName, DocumentId, DocumentPath, Embedding, PatchOperation,
    PatchSet, SimilarityMetric, VectorDimension, VectorId, VectorRecord,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collection = CollectionName::new("customers");
    let document_id = DocumentId::new("customer_123");
    let patch = PatchSet::new(vec![PatchOperation::set(
        DocumentPath::new("profile.display_name"),
        "Joshua Whalen",
    )]);

    let key = CacheKey::builder()
        .namespace(CacheNamespace::new("reviews"))
        .segment("google-business-profile")
        .segment("location")
        .segment("fort-wayne")
        .segment("summary")
        .build();

    let record = VectorRecord::new(
        VectorId::new("review_789_embedding"),
        Embedding::new(vec![0.012, -0.032, 0.481]),
    )
    .with_dimension(VectorDimension::new(3))?
    .with_similarity_metric(SimilarityMetric::Cosine);

    assert_eq!(collection.as_str(), "customers");
    assert_eq!(document_id.as_str(), "customer_123");
    assert_eq!(patch.operations().len(), 1);
    assert_eq!(
        key.to_string(),
        "reviews:google-business-profile:location:fort-wayne:summary"
    );
    assert_eq!(record.dimension(), Some(VectorDimension::new(3)));

    Ok(())
}
